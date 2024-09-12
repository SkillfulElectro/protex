use std::io::Error;
use std::ops::Deref;


/// ProTex (process mutex) is used to bring concurrency to multi processing well it can be used 
/// for multithreading too
pub struct Protex {
    mutex : std::sync::Mutex<()> , 
}


fn print_last_error() {
    let error_message = Error::last_os_error();
    eprintln!("Error: {}", error_message);
}

impl Protex {
    /// in Android and iOS API because we do not have processes in a way we do 
    /// in Linux , MacOS , Windows and etc , the name and max_lock_count are meaningless for now 
    /// in Android and iOS it uses std::sync::Mutex as backbone
    pub fn new(_name : std::ffi::CString , _max_lock_count : u32) -> Option<Self> {
        Some(Self {
            mutex : std::sync::Mutex::new(()),
        })
    }

    /// locking for use of current process or thread 
    pub fn lock(&mut self) -> Result<ProtexGuard , i32> {
        match self.mutex.lock(){
            Ok(gaurdian) => return Ok(ProtexGuard{ _guard : gaurdian}),
            Err(e) => {
                eprintln!("{:#?}" , e);
                print_last_error();
                return Err(-1);
            }
        }
    }


    fn close(&mut self) {}

    /// this function must be used when all processes finished using 
    /// the Protex
    pub fn remove(&mut self){}
}

/// automatically unlocker
pub struct ProtexGuard<'a>{
    _guard : std::sync::MutexGuard<'a,()> ,
}

impl<'a> Deref for ProtexGuard<'a> {
    type Target = ();

    fn deref(&self) -> &Self::Target {
        &()
    }
}
