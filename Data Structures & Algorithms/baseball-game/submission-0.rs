impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut result = Vec::new();
        for oper in operations{
            match oper.as_str() {
                "C" => {
                    result.pop();
                },
                "D" => {
                    if let Some(el) = result.last(){
                        result.push(el*2);
                    }
                },
                "+" => {
                    if result.len() >= 2 {
                        let a = result[result.len()-2];
                        let b = result[result.len()-1];
                        result.push(a+b);
                    }
                },
                n => if n.parse::<i32>().is_ok(){
                    let number = n.parse::<i32>().unwrap();
                    result.push(number);
                }
            }
        }

        result.iter().sum()
    }
}
