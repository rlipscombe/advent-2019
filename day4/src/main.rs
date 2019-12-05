fn main() {
    let start = 197487;
    let end = 673251;
    let mut count = 0;

    for x in start..=end {
        let s = x.to_string();
        if (s.contains("00") && !s.contains("000"))
            || (s.contains("11") && !s.contains("111"))
            || (s.contains("22") && !s.contains("222"))
            || (s.contains("33") && !s.contains("333"))
            || (s.contains("44") && !s.contains("444"))
            || (s.contains("55") && !s.contains("555"))
            || (s.contains("66") && !s.contains("666"))
            || (s.contains("77") && !s.contains("777"))
            || (s.contains("88") && !s.contains("888"))
            || (s.contains("99") && !s.contains("999"))
        {
            let unsort: Vec<char> = s.chars().collect();
            let mut sorted: Vec<char> = s.chars().collect();
            sorted.sort();
            if unsort == sorted {
                println!("{}", s);
                count += 1;
            }
        }
    }

    println!("{}", count);
}
