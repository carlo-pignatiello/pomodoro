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
                if self.time_left > 0 {
                    self.time_left -= 1;
                }
                if self.time_left == 0 {
                    self.completed_pomodoros += 1;
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
            button(
                text(label)
                    .size(16)
                    .width(Fill)
                    .align_x(iced::alignment::Horizontal::Center)
                    .color(if is_active {
                        Color::from_rgb8(239, 68, 68)
                    } else {
                        white
                    }),
            )
            .padding([10, 18])
            .width(Fill)
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
                        radius: 14.0.into(),
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
            .spacing(4)
            .width(Fill),
        )
        .padding(4)
        .width(Fill)
        .style(move |_theme| container::Style {
            background: Some(Background::Color(white_20)),
            border: Border {
                radius: 18.0.into(),
                ..Border::default()
            },
            ..container::Style::default()
        });

        // Timer card
        let icon_str = if self.is_running { "pause" } else { "play" };
        let play_pause_icon = fa_icon_solid(icon_str);

        let play_btn = {
            let b = button(play_pause_icon.size(28.0).color(bg))
                .padding([14, 24])
                .style(move |_theme, state| button::Style {
                background: Some(Background::Color(
                    if state == ButtonStatus::Disabled { white_30 } else { white },
                )),
                border: Border {
                    radius: 40.0.into(),
                    ..Border::default()
                },
                text_color: bg,
                ..button::Style::default()
            });
            if self.editing_duration { b } else { b.on_press(Message::ToggleTimer) }
        };

        let reset_btn = {
            let b = button(fa_icon_solid("rotate").size(28.0).color(white))
                .padding([14, 24])
                .style(move |_theme, state| button::Style {
                    background: Some(Background::Color(
                        if state == ButtonStatus::Disabled { white_30 } else { white_20 },
                    )),
                    border: Border {
                        radius: 40.0.into(),
                        color: white_30,
                        width: 1.0,
                    },
                    text_color: white,
                    ..button::Style::default()
                });
            if self.editing_duration { b } else { b.on_press(Message::ResetTimer) }
        };

        let duration_row: Element<Message> = if self.editing_duration {
            text_input("MM:SS", &self.duration_input)
                .on_input(Message::DurationInputChanged)
                .on_submit(Message::DurationInputSubmitted)
                .size(92)
                .line_height(iced::widget::text::LineHeight::Relative(1.0))
                .width(iced::Length::Fixed(255.0))
                .align_x(iced::alignment::Horizontal::Center)
                .padding([0, 0])
                .style(move |_theme, _status| ti::Style {
                    background: Background::Color(Color::TRANSPARENT),
                    border: Border {
                        radius: 0.0.into(),
                        color: Color::TRANSPARENT,
                        width: 0.0,
                    },
                    icon: white_30,
                    placeholder: white_30,
                    value: white,
                    selection: white_20,
                })
                .into()
        } else {
            button(
                text(self.time_string())
                    .size(92)
                    .line_height(iced::widget::text::LineHeight::Relative(1.0))
                    .color(white)
                    .width(iced::Length::Fill)
                    .align_x(iced::alignment::Horizontal::Center),
            )
                .on_press(Message::EditDuration)
                .padding([0, 0])
                .width(iced::Length::Fixed(255.0))
                .style(|_theme, _state| button::Style {
                    background: None,
                    ..button::Style::default()
                })
                .into()
        };

        let timer_card = container(
            column![
                text(self.mode_label()).size(24).color(white),
                duration_row,
                row![play_btn, reset_btn].spacing(20),
            ]
            .spacing(20)
            .align_x(iced::Alignment::Center),
        )
        .padding([40, 56])
        .width(Fill)
        .style(move |_theme| container::Style {
            background: Some(Background::Color(white_20)),
            border: Border {
                radius: 28.0.into(),
                ..Border::default()
            },
            ..container::Style::default()
        })
        .align_x(iced::alignment::Horizontal::Center);

        // Session dots
        let dot = |filled: bool| {
            let dot_color = if filled { white } else { white_30 };
            container(text(""))
                .width(20)
                .height(20)
                .style(move |_theme| container::Style {
                    background: Some(Background::Color(dot_color)),
                    border: Border {
                        radius: 10.0.into(),
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
        .spacing(10);

        let session_section = column![
            text("Sessions Completed").size(18).color(white),
            dots_row,
            text(format!("Total: {} pomodoros", self.completed_pomodoros))
                .size(16)
                .color(white_30),
        ]
        .spacing(16)
        .align_x(iced::Alignment::Center);

        // Main layout
        let content = column![mode_row, timer_card, session_section]
            .spacing(40)
            .padding([0, 28])
            .max_width(520)
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
