use std::path::Path;
use std::fs;
use std::io;

#[derive(Debug)]
pub struct RCV<'a> {
    tests_template_path: &'a Path,
    test_directory_path: &'a Path,
}

impl<'a> RCV<'a> {
    pub fn new(test_directory_path: &'a Path, tests_template_path: &'a Path) -> Result<Self,&'a str>
    {
        if !test_directory_path.exists(){
            fs::DirBuilder::new().create(test_directory_path).expect("failed creting a new test directory")
        }else{
            return Err("Invalid test_directory_path")
        }

        if !tests_template_path.exists(){
            return Err("Invalid test template path");
        }

        Ok(Self{
            tests_template_path,
            test_directory_path,
        })
    }
    

    pub fn list_tests(&self)
    {
        let paths = fs::read_dir(self.test_directory_path).expect("invalid tests dir");

        for path in paths{
            println!("{}",path.ok().unwrap().path().display())
        }
    }

    pub fn add_test(&self, name: &'a str) -> io::Result<()> {
        fn copy_dir(src: &Path, dest: &Path) -> io::Result<()> {
            // Create the destination directory
            fs::create_dir_all(dest)?;

            // Read the contents of the source directory
            for entry in fs::read_dir(src)? {
                let entry = entry?;
                let entry_path = entry.path();
                let dest_path = dest.join(entry.file_name());

                if entry_path.is_dir() {
                    // Recursively copy subdirectories
                    copy_dir(&entry_path, &dest_path)?;
                } else {
                    // Copy files
                    fs::copy(entry_path, dest_path)?;
                }
            }
            Ok(())
        }

        let paths = fs::read_dir(self.test_directory_path).expect("invalid tests dir");
        let dest = std::path::Path::new(name);

        for path in paths{
            if path.ok().unwrap().path() == dest{
                return Err(io::Error::new(io::ErrorKind::AddrInUse, "Test already exist"));
            }
        }


        copy_dir(self.test_directory_path, dest)
    }

    pub fn rem_test(&self, name: &'a str) -> io::Result<()>{
        let test_path = self.test_directory_path.join(name);
        println!("removing test path: {}", test_path.display());
        std::fs::remove_dir_all(test_path)
    }

    pub fn run_tests(&self, test_list: Option<&'a Vec<String>>, skip_list: Option<&'a Vec<String>> ) {
        todo!()
    }
}
