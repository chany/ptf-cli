use std::process::exit;

#[derive(Debug)]
// 발생가능한 에러들
pub enum Errcode {
    PlottersError(u8),
}

impl Errcode {
    pub fn get_retcode(&self) -> i32 {
        1 // 0이 아니면 모두 에러로 간주
    }
}

pub fn exit_with_retcode(res: Result<(), Errcode>) {
    match res {
        // 성공이면 0 반환
        Ok(_) => {
            exit(0);
        }
        // 에러있으면, 에러 메시지 출력하고 retcode 반환
        Err(e) => {
            let retcode = e.get_retcode();
            exit(retcode);
        }
    }
}
