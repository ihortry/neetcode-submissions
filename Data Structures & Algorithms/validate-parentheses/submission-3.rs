impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut cont = Vec::with_capacity(s.len());
        for n in s.chars(){
            match n {
                '['|'{'|'(' => {
                    cont.push(n);
                },
                ']' => {
                    if cont.pop() != Some('['){
                        return false;
                    }
                },
                '}' => {
                    if cont.pop() != Some('{'){
                        return false;
                    }
                },
                ')' => {
                    if cont.pop() != Some('('){
                        return false;
                    }
                },
                _ => {},
            }
        }
        
        cont.is_empty()
    }
}
