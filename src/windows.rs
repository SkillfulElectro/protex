use std::io::Error;
use windows::Win32::System::Threading::{
    CreateSemaphoreA, ReleaseSemaphore, WaitForSingleObject, INFINITE,
};
use windows::Win32::Foundation::{HANDLE, CloseHandle, BOOL};
use windows::core::PCSTR;

/// ProTex (process mutex) is used to bring concurrency to multi processing well it can be used 
/// for multithreading too
pub struct Protex {
    name: std::ffi::CString,
    sem: Option<HANDLE>,
}

fn print_last_error() {
    let error_message = Error::last_os_error();
    eprintln!("Error: {}", error_message);
}

impl Protex {
    /// with this function you can create new instance of ProTex
    /// writing names in /name_of_protex format has been tested
    pub fn new(name: std::ffi::CString, max_lock_count: u32) -> Option<Self> {
        let name_as_pcstr = unsafe { PCSTR(name.as_ptr().cast()) };

        let sem = unsafe {
            CreateSemaphoreA(
                None, // default security attributes
                0, // initial count
                max_lock_count, // maximum count
                name_as_pcstr // name
            )
        };

        if sem.is_invalid() {
            print_last_error();
            eprintln!("Protex creation failed");
            return None;
        }

        Some(Self {
            name,
            sem: Some(sem),
        })
    }

    /// locks the ProTex for current thread in the process , if its locked 
    /// other threads must wait till it be released 
    /// ```rust
    /// let guard = protex_var.lock().unwrap()
    /// ```
    pub fn lock(&self) -> Result<ProtexGuard , i32> {
        match self.sem {
            None => {
                eprintln!("empty Protex, use Protex::new function");
                Err(-2)
            },
            Some(sem) => unsafe {
                let wait_result = WaitForSingleObject(sem, INFINITE);
                match wait_result {
                    windows::Win32::System::Threading::WAIT_OBJECT_0 => Ok(ProtexGuard{
                        sem : self.sem ,
                    }),
                    _ => {
                        print_last_error();
                        // Handle failure by closing the semaphore and returning error code
                        self.close();
                        Err(-1)
                    }
                }
            }
        }
    }


    /// Closes the semaphore handle and sets the handle to None.
    fn close(&mut self) {
        if let Some(sem) = self.sem.take() {
            unsafe {
                CloseHandle(sem);
            }
            self.sem = None; // Ensure the handle is set to None after closing
        }
    }

    /// this function must be used when all processes finished using 
    /// the Protex
    // in windows you dont need to call it 
    pub fn remove(&mut self) {
        self.close();
    }
}

impl Drop for Protex {
    fn drop(&mut self) {
        self.close();
    }
}

/// it will release the ProTex when it goes out 
/// of the scope automatically
pub struct ProtexGuard{
    sem : Option<HANDLE> ,
}

impl ProtexGuard {
    fn unlock(&self) -> i32 {
        match self.sem {
            None => {
                eprintln!("empty Protex, use Protex::new function");
                -2
            },
            Some(sem) => unsafe {
                let result = ReleaseSemaphore(sem, 1, None);
                if result == BOOL::from(true) {
                    0
                } else {
                    print_last_error();
                    
                    self.close();
                    -1
                }
            }
        }
    }

    fn close(&mut self) {
        if let Some(sem) = self.sem.take() {
            unsafe {
                CloseHandle(sem);
            }
            self.sem = None; // Ensure the handle is set to None after closing
        }
    }
}

impl Drop for ProtexGuard {
    fn drop(&mut self){
        eprintln!("unlocking the ProTex error : {} " , self.unlock());
    }
}
