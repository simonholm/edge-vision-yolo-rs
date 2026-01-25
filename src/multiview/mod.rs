use opencv::{core::Rect, prelude::*};

pub fn split_views(frame: &Mat, n: i32) -> Vec<Mat> {
    let size = frame.size().unwrap();
    let step = size.width / n;

    let mut views = Vec::new();
    for i in 0..n {
        let roi = Rect::new(i * step, 0, step, size.height);
        let view = Mat::roi(frame, roi).unwrap();
        views.push(view);
    }
    views
}
