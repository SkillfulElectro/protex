use std::io::Error;

/// ProTex (process mutex) is used to bring concurrency to multi processing well it can be used 
/// for multithreading too
pub struct Protex {
    name : std::ffi::CString ,
    sem : Option<*mut libc::sem_t> ,
}


fn print_last_error() {
    let error_message = Error::last_os_error();
    eprintln!("Error: {}", error_message);
}

impl Protex {
    /// with this function you can create new instance of ProTex
    /// writing names in /name_of_protex has been tested
    pub fn new(name : std::ffi::CString , max_lock_count : u32) -> Option<Self> {
        unsafe {
            #[cfg(not(target_os = "macos"))]
            let sem = libc::sem_open(name.as_ptr() , libc::O_CREAT, libc::S_IRUSR | libc::S_IWUSR, max_lock_count);
            #[cfg(any(target_os = "macos"))]
            let sem = libc::sem_open(name.as_ptr() , libc::O_CREAT, (libc::S_IRUSR | libc::S_IWUSR) as libc::c_uint , max_lock_count);

            if sem == libc::SEM_FAILED {
                print_last_error();
                eprintln!("Protex creation failed");
                return None;
            }

            Some(Self {
                name : name ,
                sem : Some(sem),
            })
        }
    }
    
    /// locks the ProTex for current thread in the process , if its locked 
    /// other threads must wait till it be released 
    /// ```rust
    /// let guard = protex_var.lock().unwrap()
    /// ```
    pub fn lock(&mut self) -> Result<ProtexGuard , i32> {
        unsafe{
            match self.sem {
                None => {
                    eprintln!("empty Protex , use Protex::new function");
                    return Err(-2);
                },
                Some(sem) => {
                    if libc::sem_wait(sem) == -1 {
                        print_last_error();
                        self.close();

                        return Err(-1);
                    }
                }
            }

            return Ok(ProtexGuard{
                sem : self.sem ,
            });
        }
    }


    fn close(&mut self) {
        unsafe{
            match self.sem {
                None => {
                    return;
                },
                Some(sem) => {
                    libc::sem_close(sem);
                    self.sem = None;
                },
            }
        }
    }

    /// this function must be used when all processes finished using 
    /// the Protex
    pub fn remove(&mut self){
        self.close();
        unsafe{
            if libc::sem_unlink(self.name.as_ptr()) == -1 {
                print_last_error();
            }
        }
    }
}

impl Drop for Protex {
    fn drop(&mut self){
        self.close();
    }
}

/// it will release the ProTex when it goes out 
/// of the scope automatically
pub struct ProtexGuard{
    sem : Option<*mut libc::sem_t> ,
}

impl ProtexGuard {
    fn unlock(&mut self) -> i32 {
        unsafe{
            match self.sem {
                None => {
                    eprintln!("empty Protex , use Protex::new function");
                    return -2;
                },
                Some(sem) => {
                    if libc::sem_post(sem) == -1 {
                        print_last_error();
                        self.close();

                        return -1;
                    }
                }
            }

            return 0;
        }
    }

    fn close(&mut self) {
        unsafe{
            match self.sem {
                None => {
                    return;
                },
                Some(sem) => {
                    libc::sem_close(sem);
                    self.sem = None;
                },
            }
        }
    }

}

impl Drop for ProtexGuard {
    fn drop(&mut self){
        let res = self.unlock();
        if res == 0 {return;}
        eprintln!("unlocking the ProTex error : {} " , res);
    }
}
