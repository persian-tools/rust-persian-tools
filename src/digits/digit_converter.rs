
pub(super) fn digit_converter<S>(inp: S, from: [char; 10], to: [char; 10]) -> String where S: ToString{
    let inp: String = inp.to_string();
    
    let mut result = String::new();
    for ch in inp.chars(){
        if let Some(index) = from.iter().position(|x| { x == &ch }){
            result.push( to[index]);
        }else{
            result.push(ch);
        }
    }
    result
}