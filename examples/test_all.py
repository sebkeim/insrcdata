"""
Run all test on main application and samples

"""

import os, subprocess, sys

PATH = ".."
INSRCDATA = f"{PATH}/target/debug/insrcdata"


RESET = "reset" in sys.argv

# ==================================================================================================================
#
# ==================================================================================================================
def test_root():
      # build
      r = os.system(f"cargo build --manifest-path {PATH}/Cargo.toml")
      assert r==0, f"failed cargo build : root"
     
      # unitest
      r = os.system(f"cargo test --manifest-path {PATH}/Cargo.toml")
      assert r==0, f"failed cargo unitest : root"
        
     
      # fmt
      r = os.system(f"cargo fmt --check --manifest-path {PATH}/Cargo.toml")
      assert r==0, f"failed cargo fmt : root"
     
      # clippy
      r = os.system(f"cargo clippy --manifest-path {PATH}/Cargo.toml")
      assert r==0, f"failed cargo clippy : root"
      
      # TODO: clippy warinings should produce an error
     
 
# ==================================================================================================================
# Test sample directory
# ==================================================================================================================

class sample:
 
      def __init__(self, name):
            self.name = name
            
            
      def project_path(self):
            return f"{PATH}/examples/{self.name}"
      
      def lang_path(self):
            return f"{self.project_path()}/{self.LANG}-{self.name}"
      
            
      def regress_path(self):
            return f"{self.lang_path()}/regression"
      
      def product_path(self):
            raise NotImplemented
              
      def build(self):
            raise NotImplemented
            
            
      def build_and_run(self):
            """return stdout content"""
      
            # force regeneration
            try:
                  os.remove(f"{self.project_path()}/insrcdata/{self.dest()}")
            except FileNotFoundError:
                   
                  pass
      
            # generate insrcdata source
            r = os.system(f"{INSRCDATA}  {self.project_path()}/insrcdata/insrcdata.toml --dest {self.dest()}")
            assert r==0, f"failed processing insrcdata {self.name}"
            
            # build
            self.build()
           
            # commands
            cmd  =  subprocess.Popen([f"{self.product_path()}"], stdout=subprocess.PIPE)
            output = cmd.communicate()[0]
            assert cmd.returncode==0, f"failed run : {self.name}"
            return output
      
      
      def get_template(self):
            """return generated template for trait"""
            
            cmd  =  subprocess.Popen([INSRCDATA, f"{self.project_path()}/insrcdata/insrcdata.toml", "--interface", "--dest", self.dest() ], stdout=subprocess.PIPE)
            output = cmd.communicate()[0]
            assert cmd.returncode==0, f"failed insrcdata interface : {self.name}"
            return output
      
      
      def check_regression(self, new_output, consigne_name):
            path = f"{self.regress_path()}/{consigne_name}.txt"
            try:
                  if RESET:
                        os.remove(path)
                  consigne = open(path, "rb").read()
            except FileNotFoundError:
                  print(f"#warning: writing {self.name} {consigne_name} regression; test skiped")
                  open(path, "wb").write(new_output)
            else:
                  if new_output!=consigne:
                        open(f"{self.regress_path()}/{consigne_name}_new.txt", "wb").write(new_output)
                        assert False, f"output differ from regression : {self.name} {consigne_name}"

      def test(self):
      
            output = self.build_and_run()
            
            try:
                  os.mkdir(self.regress_path())
            except FileExistsError:
                  pass
                  
            
            # sample output
            
            self.check_regression(output, "output")

            # trait template generation
            template = self.get_template()
            self.check_regression(template, "template")
            
                 
      
       
       

class sample_rust(sample):
      LANG = "rust"
      
      def dest(self):
            return f"../rust-{self.name}/src/insrcdata.rs"
      
  
            
      def regress_path(self):
            return f"{self.lang_path()}/target/debug/regression"


      def product_path(self):
            return f"{self.lang_path()}/target/debug/{self.name}"


   
      def build(self):
           
            # build
            r = os.system(f"cargo build --manifest-path {self.lang_path()}/Cargo.toml")
            assert r==0, f"failed cargo build : {self.name}"
        
   
      
      def test(self):
            sample.test(self)
            
            # fmt
            #r = os.system(f"cargo fmt --check --manifest-path {self.project_path()}/Cargo.toml")
            #assert r==0, f"failed cargo fmt : root"
            #TODO: fmt on project will need to make generated innexata.rs compliant
         
     
            # clippy
            r = os.system(f"cargo clippy --manifest-path {self.lang_path()}/Cargo.toml")
            assert r==0, f"failed cargo clippy : root"



class sample_c(sample):
      LANG = "c"
      
      def dest(self):
            return f"../c-{self.name}/insrcdata.c"
    
      def regress_path(self):
            return f"{self.lang_path()}/target/regression"


      def product_path(self):
            return f"{self.lang_path()}/target/{self.name}"


   
      def build(self):
      
            try:
                  os.mkdir(f"{self.lang_path()}/target")
            except FileExistsError:
                  pass
           
            # build
            r = os.system(f"cd {self.lang_path()}; cc main.c insrcdata.c -o ./target/{self.name} ")
            assert r==0, f"failed cc build : {self.name}"
        
   

def test_sample(name):
       
      r = sample_rust(name)
      r.test()
      c = sample_c(name)
      c.test()


def test_examples():
      sample_path = f"{PATH}/examples"
      for name in os.listdir(sample_path):
            print(f"{sample_path}/{name}/Cargo.toml")
            if os.path.exists(f"{sample_path}/{name}/rust-{name}/Cargo.toml"):
                  print(f"\n   -- sample {name} --")
                  test_sample(name)
                  

# ==================================================================================================================
#
# ==================================================================================================================
test_root()
test_examples()

