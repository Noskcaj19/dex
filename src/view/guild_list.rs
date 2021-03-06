use model::Context;
use view::terminal::Terminal;

use std::collections::HashMap;
use std::sync::Arc;

use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::CACHE;
use termbuf::Style;
use termbuf::TermSize;

const TOP_START: usize = 2;
const LEFT_START: usize = 2;
const MAX_LEN: usize = 28;

#[derive(Debug)]
struct GuildEntry {
    pub guild: Arc<RwLock<Guild>>,
    pub categories: HashMap<ChannelId, CategoryEntry>,
    pub misc: Vec<Arc<RwLock<GuildChannel>>>,
}

#[derive(Debug)]
struct CategoryEntry {
    category: Arc<RwLock<GuildChannel>>,
    channels: Vec<Arc<RwLock<GuildChannel>>>,
}

impl CategoryEntry {
    pub fn new(category: Arc<RwLock<GuildChannel>>) -> CategoryEntry {
        CategoryEntry {
            category,
            channels: Vec::new(),
        }
    }
}

impl GuildEntry {
    pub fn new(guild: Arc<RwLock<Guild>>) -> GuildEntry {
        GuildEntry {
            guild,
            categories: HashMap::new(),
            misc: Vec::new(),
        }
    }
}

fn truncate(s: String, new_len: usize) -> String {
    if s.len() < new_len {
        s
    } else {
        s.chars()
            .take(new_len.saturating_sub(1))
            .collect::<String>() + "…"
    }
}

pub struct GuildList {
    guild_list: Vec<GuildEntry>,
}

impl GuildList {
    pub fn new() -> GuildList {
        GuildList {
            guild_list: Vec::new(),
        }
    }

    pub fn populate_guild_list(&mut self) {
        let user = {
            let cache = CACHE.read();
            &cache.user.clone()
        };

        self.guild_list.clear();

        if let Ok(guilds) = user.guilds() {
            for guild in guilds {
                if let Some(full_guild) = guild.id.find() {
                    let mut guild = GuildEntry::new(full_guild.clone());

                    // TODO: Switch to entry api
                    for raw_channel in full_guild.read().channels.values() {
                        let channel = raw_channel.read();

                        if let ChannelType::Category = channel.kind {
                            guild
                                .categories
                                .insert(channel.id, CategoryEntry::new(raw_channel.clone()));
                        }
                    }

                    for raw_channel in full_guild.read().channels.values() {
                        let channel = raw_channel.read();

                        if let Ok(perms) = channel.permissions_for(user.id) {
                            if !perms.send_messages() {
                                continue;
                            }
                        }
                        match channel.kind {
                            ChannelType::Category => continue,
                            _ => if let Some(category_id) = channel.category_id {
                                if let Some(mut category) = guild.categories.get_mut(&category_id) {
                                    category.channels.push(raw_channel.clone());
                                } else {
                                    guild.misc.push(raw_channel.clone());
                                }
                            } else {
                                guild.misc.push(raw_channel.clone());
                            },
                        }
                    }
                    self.guild_list.push(guild);
                }
            }
        }

        for guild in &mut self.guild_list {
            for category in guild.categories.values_mut() {
                category.channels.sort_by_key(|chan| chan.read().position);
            }
            guild.misc.sort_by_key(|misc| misc.read().position);
        }
    }

    pub fn render(&self, screen: &mut Terminal, size: TermSize, context: &Arc<RwLock<Context>>) {
        let mut y = 0;
        let max_y = size.height.saturating_sub(7);
        let current_channel = context.read().channel;
        for guild in &self.guild_list {
            if y >= max_y {
                break;
            }
            screen
                .buf
                .string_builder(
                    LEFT_START,
                    TOP_START + y,
                    &truncate(
                        guild.guild.read().name.clone(),
                        MAX_LEN.saturating_sub(LEFT_START + 2),
                    ),
                )
                .style(Style::Bold)
                .draw();
            y += 1;

            let mut categories = guild.categories.values().collect::<Vec<_>>();
            categories.sort_by_key(|entry| entry.category.read().position);

            for category in &categories {
                if y >= max_y {
                    break;
                }
                screen.buf.print(
                    LEFT_START + 2,
                    TOP_START + y,
                    &truncate(
                        category.category.read().name.clone(),
                        MAX_LEN.saturating_sub(LEFT_START + 4),
                    ),
                );
                y += 1;
                for channel in &category.channels {
                    if y >= max_y {
                        break;
                    }
                    let channel = channel.read();
                    let mut text =
                        truncate(channel.name.clone(), MAX_LEN.saturating_sub(LEFT_START + 7));
                    if let ChannelType::Voice = channel.kind {
                        text = format!("{} {}", context.read().char_set.volume_off(), text);
                    }
                    if Some(channel.id) == current_channel {
                        screen
                            .buf
                            .string_builder(LEFT_START + 5, TOP_START + y, &text)
                            .style(Style::Bold)
                            .draw();
                    } else {
                        screen.buf.print(LEFT_START + 5, TOP_START + y, &text);
                    }
                    y += 1;
                }
            }
            for misc in &guild.misc {
                if y >= max_y {
                    break;
                }
                screen.buf.print(
                    LEFT_START + 2,
                    TOP_START + y,
                    &truncate(
                        misc.read().name.clone(),
                        MAX_LEN.saturating_sub(LEFT_START + 4),
                    ),
                );
                y += 1;
            }
        }

        screen
            .buf
            .draw_vertical_line(MAX_LEN - 1, 1, size.height.saturating_sub(6))
    }
}
