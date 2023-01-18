use fltk::{prelude::*, *};

fn main() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    let flex = group::Flex::default().with_size(200, 200).column().center_of_parent();
    let label = frame::Frame::default().with_label("Enter weight(kg)");
    let input = input::FloatInput::default();
    let label = frame::Frame::default().with_label("Enter height(cm)");
    let htin = input::FloatInput::default();
    let mut btn = button::Button::default().with_label("BMI Calculate");
    let mut bmilb = frame::Frame::default().with_label("BMI ");
    let mut stlb = frame::Frame::default().with_label("LEVEL ");
    flex.end();
    win.end();
    win.show();

    btn.set_callback(move |btn| {
        let (bmi, status) = console_cmd(input.value().parse::<f64>().unwrap() , htin.value().parse::<f64>().unwrap());
        let bstr = format!("BMI {:.2}", bmi);
        bmilb.set_label(&bstr);
        stlb.set_label(&format!("{}", status.to_string()));
    });

    a.run().unwrap();
}
fn console_cmd(weight: f64, height: f64)-> (f64, &'static str) {
    //let mut weight = 0.0;
    //let mut height  = 0.0;

    println!("Your weight and height: {}, {}", weight, height);

    let (bmi, status) = bmi_calculate(weight, height);
    println!("BMI 身体质量指数: {}", bmi);
    println!("BODY CONDITION 身体状况: {}", status);
    (bmi, status)
}

fn bmi_calculate(weight: f64, height: f64) -> (f64, &'static str) {
    let height: f64 = height as f64 / 100.0;
    let bmi = weight as f64 / height / height;

    let status = match bmi {
        _ if bmi < 18.5 => "体重过轻",
        _ if bmi < 24.0 => "体重正常",
        _ if bmi < 28.0 => " 亚肥胖 ",
        _ if bmi < 30.0 => "LEVEL 1 肥胖一级",
        _ if bmi < 40.0 => "LEVEL 2 肥胖二级",
        _ if bmi.is_finite() => "LEVEL 3 肥胖三级",
        _ => "输入错误",
    };

    (bmi, status)
}