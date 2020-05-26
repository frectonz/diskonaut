use ::tui::layout::Rect;
use ::tui::terminal::Frame;
use ::tui::backend::Backend;
use ::tui::layout::{Layout, Constraint, Direction};
use ::tui::widgets::Widget;

use crate::ui::title::BasePath;
use crate::ui::title::CurrentPath;
use crate::ui::title::SpaceFreed;
use crate::ui::FolderInfo;

fn three_part_layout (first_part_len: u16, second_part_len: u16, third_part_len: u16, rect: Rect) -> (Option<Rect>, Option<Rect>, Option<Rect>) {
    if first_part_len + second_part_len + third_part_len <= rect.width {
        let remainder = rect.width - first_part_len - second_part_len - third_part_len;
        let parts = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Length(first_part_len),
                    Constraint::Length(second_part_len + remainder),
                    Constraint::Length(third_part_len),
                ].as_ref()
            )
            .split(rect);
        (Some(parts[0]), Some(parts[1]), Some(parts[2]))
    } else if second_part_len + third_part_len <= rect.width {
        let remainder = rect.width - second_part_len - third_part_len;
        let parts = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Length(second_part_len + remainder),
                    Constraint::Length(third_part_len),
                ].as_ref()
            )
            .split(rect);

        (Some(parts[0]), Some(parts[1]), None)
    } else {
        (Some(rect), None, None)
    }
}

fn two_part_layout (first_part_len: u16, second_part_len: u16, rect: Rect) -> (Option<Rect>, Option<Rect>) {
    if first_part_len + second_part_len <= rect.width {
        let remainder = rect.width - first_part_len - second_part_len;
        let parts = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Length(first_part_len),
                    Constraint::Length(second_part_len + remainder),
                ].as_ref()
            )
            .split(rect);
        (Some(parts[0]), Some(parts[1]))
    } else {
        (Some(rect), None)
    }
}

pub struct TitleLine <'a> {
    base_path_info: FolderInfo<'a>,
    current_path_info: FolderInfo<'a>,
    space_freed: u64,
    show_loading: bool,
    scanning_visual_indicator: u64,
    frame_around_current_path: bool,
    frame_around_space_freed: bool,
    current_path_is_red: bool,
}

impl <'a>TitleLine<'a> {
    pub fn new(base_path_info: FolderInfo<'a>, current_path_info: FolderInfo<'a>, space_freed: u64) -> Self {
        Self {
            base_path_info,
            current_path_info,
            space_freed,
            scanning_visual_indicator: 0,
            show_loading: false,
            frame_around_current_path: false,
            frame_around_space_freed: false,
            current_path_is_red: false,
        }
    }
    pub fn show_loading(mut self) -> Self {
        self.show_loading = true;
        self
    }
    pub fn frame_around_current_path(mut self, frame_around_current_path: bool) -> Self {
        self.frame_around_current_path = frame_around_current_path;
        self
    }
    pub fn frame_around_space_freed(mut self, frame_around_space_freed: bool) -> Self {
        self.frame_around_space_freed = frame_around_space_freed;
        self
    }
    pub fn current_path_is_red(mut self, current_path_is_red: bool) -> Self {
        self.current_path_is_red = current_path_is_red;
        self
    }
    pub fn scanning_visual_indicator(mut self, scanning_visual_indicator: u64) -> Self {
        self.scanning_visual_indicator = scanning_visual_indicator;
        self
    }
    pub fn render(&self, frame: &mut Frame<impl Backend>, rect: Rect) {
        let mut base_path = BasePath::new(&self.base_path_info.path, self.base_path_info.size, self.base_path_info.num_descendants)
            .loading(self.show_loading)
            .visual_indicator(self.scanning_visual_indicator);
        let current_path = CurrentPath::new(&self.current_path_info.path, self.current_path_info.size, self.current_path_info.num_descendants)
            .frame(self.frame_around_current_path)
            .red(self.current_path_is_red);
        let space_freed = SpaceFreed::new(self.space_freed)
            .frame(self.frame_around_space_freed);

        let min_current_path_len = current_path.len() as u16 + 10;
        let min_base_path_len = base_path.len() as u16 + 10;
        let min_space_freed_text_len = space_freed.len() as u16 + 10;

        if self.show_loading {
            let layout_parts = two_part_layout(min_base_path_len, min_current_path_len, rect);
            match layout_parts {
                (Some(left), Some(right)) => {
                    base_path.render(frame, left);
                    current_path.render(frame, right);
                },
                (Some(rect), None) => {
                    current_path.render(frame, rect);
                },
                _ => {
                    unreachable!("wrong order of layout parts");
                }
            }
        } else {
            let layout_parts = three_part_layout(min_base_path_len, min_current_path_len, min_space_freed_text_len, rect);
            match layout_parts {
                (Some(left), Some(middle), Some(right)) => {
                    base_path.render(frame, left);
                    current_path.render(frame, middle);
                    space_freed.render(frame, right);
                },
                (Some(left), Some(right), None) => {
                    current_path.render(frame, left);
                    space_freed.render(frame, right);
                }
                (Some(rect), None, None) => {
                    current_path.render(frame, rect);
                }
                _ => {
                    unreachable!("wrong order of layout parts");
                }
            }
        }
    }
}