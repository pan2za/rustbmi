fn main() {
    loop {
        let mut weight = 0.0;
        let mut height  = 0.0;
        let mut line = String::new();
        println!("Enter your weight :");
        _ = std::io::stdin().read_line(&mut line).unwrap();
        let trimmed = line.trim();
        match trimmed.parse::<f64>() {
            Ok(i) => {
                weight = i;
                println!("your weight input: {}", weight);
            }
            Err(..) => println!("weight was not an float: {}", trimmed),
        };
    
        println!("Enter your height :");
        let mut lineh = String::new();
        _ = std::io::stdin().read_line(&mut lineh).unwrap();
        let trimmedheight = lineh.trim();
        match trimmedheight.parse::<f64>() {
            Ok(i) => {
                height = i;
                println!("your height input: {}", height);
            }
            Err(..) => println!("height was not an float: {}", trimmed),
        };
    
        println!("Your weight and height: {}, {}", weight, height);
    
        let (bmi, status) = bmi_calculate(weight, height);
        println!("BMI 身体质量指数: {}", bmi);
        println!("BODY CONDITION 身体状况: {}", status);
    }
  
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