extern crate num;
use num::Complex;
use std::str::FromStr;


/// Пытается определить, принадлежэит ли `с` множеству Мендельброта, ограничившись
/// `limit` итерациями.
///
/// Если `c` не пренадлежит множеству, вернуть `Some(i)`, где `i` - число итераций,
/// понадобившихся для того чтобы, `с` покинула круг радиуса 2 с центром в начале координат.
/// Если `с` может принадлежать множеству(за `limit` итераций не удалось доказать что пренадлежит),
/// вернуть `None`.

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex {re: 0.0, im: 0.0};
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

/// Разбираем строку `s` вида <left><separator><right>
/// Если `s` удалось разобрать то возвращаем `Some<(x, y)>` в противном случае `None`.

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",        ','), None);
    assert_eq!(parse_pair::<i32>("10,",     ','), None);
    assert_eq!(parse_pair::<i32>(",10",     ','), None);
    assert_eq!(parse_pair::<i32>("30,500x", ','), None);
    assert_eq!(parse_pair::<f64>("1.2x",    'x'), None);
    assert_eq!(parse_pair::<f64>("1.5x3.4", 'x'), Some((1.5, 3.4)));
    assert_eq!(parse_pair::<i32>("20,30",   ','), Some((20, 30)));
    assert_eq!(parse_pair::<i32>("20, 30",  ','), None);
}


/// Разбирает пару чисел с плавающей точкой
/// и возвращает ее в виде комплексного числа

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex {re, im}),
        None => None
    }
}

#[test]

fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0654"), Some(Complex {re: 1.25, im: -0.0654}));
    assert_eq!(parse_complex(",-0.0654"), None);
}

fn main(){}
