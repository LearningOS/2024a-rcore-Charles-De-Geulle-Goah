//! Process management syscalls

use crate::{
    task::{exit_current_and_run_next, get_current_task_id, get_task_status, suspend_current_and_run_next},
    timer::get_time_us,
    syscall::*,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}



/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
} 

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let current_task_id = get_current_task_id();  //如何获取current_task变量
    //通过id获得状态
    let current_task_status = get_task_status(current_task_id);
    //获取任务调用系统调用的次数
    //获取运行总时长
    let info = get_task_info(current_task_id);
    unsafe {
        (*_ti).status = current_task_status;
        (*_ti).syscall_times = info.syscall_times;
        (*_ti).time = info.time;
    }
    // -1
    0
}
