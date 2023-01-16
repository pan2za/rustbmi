fn main() {
    let (weight, height) = (97.5, 185);
    let (bmi, status) = bmi_calculate(weight, height);
    println!("身体质量指数: {}", bmi);
    println!("身体状况: {}", status);
}

fn bmi_calculate(weight: f64, height: i32) -> (f64, &'static str) {
    let height: f64 = height as f64 / 100.0;
    let bmi = weight as f64 / height / height;

    let status = match bmi {
        _ if bmi < 18.5 => "体重过轻",
        _ if bmi < 24.0 => "体重正常",
        _ if bmi < 28.0 => " 亚肥胖 ",
        _ if bmi < 30.0 => "肥胖一级",
        _ if bmi < 40.0 => "肥胖二级",
        _ if bmi.is_finite() => "肥胖三级",
        _ => "输入错误",
    };

    (bmi, status)
}