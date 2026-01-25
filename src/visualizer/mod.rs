use opencv::{imgproc, prelude::*};
use anyhow::Result;

pub fn draw(frame: &Mat, boxes: &[(i32,i32,i32,i32)]) -> Result<()> {
    let mut out = frame.clone();
    for (x1,y1,x2,y2) in boxes {
        imgproc::rectangle(
            &mut out,
            (*x1, *y1, x2-x1, y2-y1),
            (0.0,255.0,0.0),
            2,
            imgproc::LINE_8,
            0
        )?;
    }
    Ok(())
}
