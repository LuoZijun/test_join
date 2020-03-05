#![feature(test)]
extern crate test;
extern crate itertools;


#[bench]
fn bench_iter_fold_format(b: &mut test::Bencher) {
    // See what landed recently in Firefox Nightly!
    let data = vec!["See", "what", "landed", "recently", "in", "Firefox", "Nightly", "!"];
    
    b.iter(|| {
        let _ = data.iter()
            .fold(String::new(), |acc, s| format!("{}{}{}", acc, ", ", s));
    })
}

#[bench]
fn bench_iter_fold_push_str(b: &mut test::Bencher) {
    let data = vec!["See", "what", "landed", "recently", "in", "Firefox", "Nightly", "!"];
    
    b.iter(|| {
        let _ = data.iter().fold(String::new(), |mut acc, s| {
            acc.push_str(", ");
            acc.push_str(s);
            acc
        });
    })
}

#[bench]
fn bench_collect_join(b: &mut test::Bencher) {
    let data = vec!["See", "what", "landed", "recently", "in", "Firefox", "Nightly", "!"];
    
    b.iter(|| {
        let _ = data.iter()
            .map(|s| &**s )
            .collect::<Vec<&str>>()
            .join(", ");
    })
}

#[bench]
fn bench_itertools(b: &mut test::Bencher) {
    let data = vec!["See", "what", "landed", "recently", "in", "Firefox", "Nightly", "!"];
    
    b.iter(|| {
        let _ = itertools::join(&data, ", ");
    })
}

