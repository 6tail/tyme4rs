# Tyme [![License](https://img.shields.io/badge/license-MIT-4EB1BA.svg?style=flat-square)](https://github.com/6tail/tyme4rs/blob/master/LICENSE)

Tyme是一个非常强大的日历工具库，可以看作 [Lunar](https://6tail.cn/calendar/api.html "https://6tail.cn/calendar/api.html") 的升级版，拥有更优的设计和扩展性，支持公历和农历、星座、干支、生肖、节气、法定假日等。

## 示例

    // install
    cargo install tyme4rs
     
    // test.rs
    use tyme4rs::tyme::solar::SolarDay;
     
    let solar: SolarDay = SolarDay::from_ymd(1986, 5, 29).unwrap();
     
    // 1986年5月29日
    println!("{}", solar.to_string());
    
    // 农历丙寅年四月廿一
    println!("{}", solar.get_lunar_day().to_string());
     

## 文档

请移步至 [https://6tail.cn/tyme.html](https://6tail.cn/tyme.html "https://6tail.cn/tyme.html")

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=6tail/tyme4rs&type=Date)](https://star-history.com/#6tail/tyme4rs&Date)
