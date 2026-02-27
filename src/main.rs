use iced::widget::{button, column, container, row, text, text_input};
use iced::widget::text_input as ti;
use iced::{Background, Border, Color, Element, Fill, Subscription, time};
use std::time::Duration;
use iced::widget::button::Status as ButtonStatus;
use iced_font_awesome::fa_icon_solid;

fn main() -> iced::Result {
    iced::application(Pomodoro::default, Pomodoro::update, Pomodoro::view)
        .title("Pomodoro")
        .subscription(Pomodoro::subscription)
        .run()
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum TimerMode {
    #[default]
    Work,
    ShortBreak,
    LongBreak,
}

struct Pomodoro {
    mode: TimerMode,
    time_left: u64,
    is_running: bool,
    completed_pomodoros: usize,
    work_duration: u64,
    short_break_duration: u64,
    long_break_duration: u64,
    editing_duration: bool,
    duration_input: String,
}

impl Default for Pomodoro {
    fn default() -> Self {
        Self {
            mode: TimerMode::Work,
            time_left: 25 * 60,
            is_running: false,
            completed_pomodoros: 0,
            work_duration: 25 * 60,
            short_break_duration: 5 * 60,
            long_break_duration: 15 * 60,
            editing_duration: false,
            duration_input: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    SwitchMode(TimerMode),
    ToggleTimer,
    ResetTimer,
    AdjustDuration(i64),
    EditDuration,
    DurationInputChanged(String),
    DurationInputSubmitted,
    Tick,
}

impl Pomodoro {
    fn duration_for(&self, mode: TimerMode) -> u64 {
        match mode {
            TimerMode::Work => self.work_duration,
            TimerMode::ShortBreak => self.short_break_duration,
            TimerMode::LongBreak => self.long_break_duration,
        }
    }

    fn set_duration_for(&mut self, mode: TimerMode, secs: u64) {
        match mode {
            TimerMode::Work => self.work_duration = secs,
            TimerMode::ShortBreak => self.short_break_duration = secs,
            TimerMode::LongBreak => self.long_break_duration = secs,
        }
    }

    fn parse_duration(input: &str) -> Option<u64> {
        let s = input.trim();
        if let Some((m, sec)) = s.split_once(':') {
            let mins: u64 = m.trim().parse().ok()?;
            let secs: u64 = sec.trim().parse().ok()?;
            if secs >= 60 { return None; }
            Some((mins * 60 + secs).max(60))
        } else {
            let mins: u64 = s.parse().ok()?;
            Some((mins * 60).max(60))
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SwitchMode(mode) => {
                self.mode = mode;
                self.time_left = self.duration_for(mode);
                self.is_running = false;
                self.editing_duration = false;
            }
            Message::ToggleTimer => {
                self.is_running = !self.is_running;
            }
            Message::ResetTimer => {
                self.time_left = self.duration_for(self.mode);
                self.is_running = false;
            }
            Message::AdjustDuration(delta_mins) => {
                let mode = self.mode;
                let current = self.duration_for(mode) as i64;
                let new_secs = (current + delta_mins * 60).max(60) as u64;
                self.set_duration_for(mode, new_secs);
                self.time_left = new_secs;
                self.is_running = false;
            }
            Message::EditDuration => {
                self.is_running = false;
                self.duration_input = self.time_string();
                self.editing_duration = true;
            }
            Message::DurationInputChanged(s) => {
                self.duration_input = s;
            }
            Message::DurationInputSubmitted => {
                if let Some(secs) = Self::parse_duration(&self.duration_input) {
                    let mode = self.mode;
                    self.set_duration_for(mode, secs);
                    self.time_left = secs;
                }
                self.editing_duration = false;
            }
            Message::Tick => {
                if self.time_left == 0 {
                    self.completed_pomodoros += 1;
                }
                if self.time_left > 0 {
                    self.time_left -= 1;
                } else {
                    self.is_running = false;
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.is_running {
            time::every(Duration::from_secs(1)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn bg_color(&self) -> Color {
        match self.mode {
            TimerMode::Work => Color::from_rgb8(239, 68, 68),
            TimerMode::ShortBreak => Color::from_rgb8(34, 197, 94),
            TimerMode::LongBreak => Color::from_rgb8(59, 130, 246),
        }
    }

    fn mode_label(&self) -> &str {
        match self.mode {
            TimerMode::Work => "Focus Time",
            TimerMode::ShortBreak => "Short Break",
            TimerMode::LongBreak => "Long Break",
        }
    }

    fn time_string(&self) -> String {
        let mins = self.time_left / 60;
        let secs = self.time_left % 60;
        format!("{:02}:{:02}", mins, secs)
    }

    fn view(&self) -> Element<'_, Message> {
        let bg = self.bg_color();
        let white = Color::WHITE;
        let white_20 = Color { a: 0.20, ..white };
        let white_15 = Color { a: 0.15, ..white };
        let white_30 = Color { a: 0.30, ..white };

        // Mode switcher buttons
        let make_mode_btn = |label: &'static str, mode: TimerMode| {
            let is_active = self.mode == mode;
            button(text(label).size(14).color(if is_active {
                Color::from_rgb8(239, 68, 68)
            } else {
                white
            }))
            .padding([8, 16])
            .on_press(Message::SwitchMode(mode))
            .style(move |_theme, state| {
                let bg_color = if is_active {
                    white
                } else if state == ButtonStatus::Hovered {
                    white_15
                } else {
                    Color::TRANSPARENT
                };
                button::Style {
                    background: Some(Background::Color(bg_color)),
                    border: Border {
                        radius: 12.0.into(),
                        ..Border::default()
                    },
                    text_color: if is_active {
                        Color::from_rgb8(239, 68, 68)
                    } else {
                        white
                    },
                    ..button::Style::default()
                }
            })
        };

        let mode_row = container(
            row![
                make_mode_btn("Pomodoro", TimerMode::Work),
                make_mode_btn("Short Break", TimerMode::ShortBreak),
                make_mode_btn("Long Break", TimerMode::LongBreak),
            ]
            .spacing(4),
        )
        .padding(4)
        .style(move |_theme| container::Style {
            background: Some(Background::Color(white_20)),
            border: Border {
                radius: 16.0.into(),
                ..Border::default()
            },
            ..container::Style::default()
        });

        // Timer card
        let icon_str = if self.is_running { "play" } else { "stop" }; 
        let play_pause_icon = fa_icon_solid(icon_str);

        let play_btn = button(play_pause_icon.size(24.0).color(bg))
            .padding([12, 20])
            .on_press(Message::ToggleTimer)
            .style(move |_theme, _state| button::Style {
                background: Some(Background::Color(white)),
                border: Border {
                    radius: 36.0.into(),
                    ..Border::default()
                },
                text_color: bg,
                ..button::Style::default()
            });

        let reset_btn = button(fa_icon_solid("rotate").size(24.0).color(white))
            .padding([12, 20])
            .on_press(Message::ResetTimer)
            .style(move |_theme, _state| button::Style {
                background: Some(Background::Color(white_20)),
                border: Border {
                    radius: 36.0.into(),
                    color: white_30,
                    width: 1.0,
                },
                text_color: white,
                ..button::Style::default()
            });

        let make_adjust_btn = |label: &'static str, delta: i64| {
            button(text(label).size(18).color(white))
                .padding([6, 14])
                .on_press(Message::AdjustDuration(delta))
                .style(move |_theme, _state| button::Style {
                    background: Some(Background::Color(white_15)),
                    border: Border {
                        radius: 10.0.into(),
                        ..Border::default()
                    },
                    text_color: white,
                    ..button::Style::default()
                })
        };

        let duration_row: Element<Message> = if self.editing_duration {
            text_input("MM:SS or minutes", &self.duration_input)
                .on_input(Message::DurationInputChanged)
                .on_submit(Message::DurationInputSubmitted)
                .size(60)
                .padding([8, 16])
                .style(move |_theme, _status| ti::Style {
                    background: Background::Color(Color::TRANSPARENT),
                    border: Border {
                        radius: 8.0.into(),
                        color: white_30,
                        width: 1.0,
                    },
                    icon: white_30,
                    placeholder: white_30,
                    value: white,
                    selection: white_20,
                })
                .into()
        } else {
            row![
                make_adjust_btn("−", -1),
                button(text(self.time_string()).size(80).color(white))
                    .on_press(Message::EditDuration)
                    .style(|_theme, _state| button::Style {
                        background: None,
                        ..button::Style::default()
                    }),
                make_adjust_btn("+", 1),
            ]
            .spacing(12)
            .align_y(iced::Alignment::Center)
            .into()
        };

        let timer_card = container(
            column![
                text(self.mode_label()).size(20).color(white),
                duration_row,
                row![play_btn, reset_btn].spacing(16),
            ]
            .spacing(16)
            .align_x(iced::Alignment::Center),
        )
        .padding([32, 48])
        .style(move |_theme| container::Style {
            background: Some(Background::Color(white_20)),
            border: Border {
                radius: 24.0.into(),
                ..Border::default()
            },
            ..container::Style::default()
        })
        .align_x(iced::alignment::Horizontal::Center);

        // Session dots
        let dot = |filled: bool| {
            let dot_color = if filled { white } else { white_30 };
            container(text(""))
                .width(16)
                .height(16)
                .style(move |_theme| container::Style {
                    background: Some(Background::Color(dot_color)),
                    border: Border {
                        radius: 8.0.into(),
                        ..Border::default()
                    },
                    ..container::Style::default()
                })
        };

        let dots_row = row![
            dot(self.completed_pomodoros > 0),
            dot(self.completed_pomodoros > 1),
            dot(self.completed_pomodoros > 2),
            dot(self.completed_pomodoros > 3),
        ]
        .spacing(8);

        let session_section = column![
            text("Sessions Completed").size(16).color(white),
            dots_row,
            text(format!("Total: {} pomodoros", self.completed_pomodoros))
                .size(14)
                .color(white_30),
        ]
        .spacing(12)
        .align_x(iced::Alignment::Center);

        // Main layout
        let content = column![mode_row, timer_card, session_section]
            .spacing(32)
            .padding([0, 24])
            .max_width(450)
            .align_x(iced::Alignment::Center);

        container(
            container(content)
                .width(Fill)
                .height(Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center),
        )
        .width(Fill)
        .height(Fill)
        .style(move |_theme| container::Style {
            background: Some(Background::Color(bg)),
            ..container::Style::default()
        })
        .into()
    }
}
