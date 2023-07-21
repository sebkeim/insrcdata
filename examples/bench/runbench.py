"""
generate benchmark data


python3 mkbench <COUNT>

"""

import os, sys, csv, random, string, timeit, time
 
PATH = "../.."
INSRCDATA = f"{PATH}/target/debug/insrcdata"

# ==================================================================================================================
#
# ==================================================================================================================
BENCH_PATH = "./insrcdata/databench.csv"
def bench_data(row_count):
      pr = random.Random(1)
      f = csv.writer(open(BENCH_PATH, 'w'))
      f.writerow(["byte","short", "int", "str"])
      for i in range(row_count):
            f.writerow([pr.randrange(0xFF),pr.randrange(0xFFFF), pr.randrange(0xFFFFFFFF), ''.join(random.choices(string.ascii_uppercase + string.digits, k=32))])
      
def time_command(cmd, count=None) :
      if count is not None:
            t = timeit.timeit(f'os.system("{cmd}")', "import os", number=count) /count
      start = time.time()
      
      r = os.system(cmd)
      assert r==0, f"failed {cmd}"
      
      if count is None:
            t = time.time() - start
      
      print(f"{int(t*1000)} ms : {cmd}")



def build_and_run(opts):
      # generate insrcdata source
      time_command(f"{INSRCDATA}  ./insrcdata/insrcdata.toml", 10)
      
      # compile time
      time_command(opts['build'])
   
      # source code size
      exe = opts['exe']
      sz = os.path.getsize(exe)
      print(f"{int(sz/1000)} ko : executable size")
     
      # measure start time
      time_command(f"{exe} startup", 100)
      
      # run benchmark
      time_command(f"{exe} bench")
  

LANGS = {
      'rust' : {
            'build':"cargo build -q --release --manifest-path rust-bench/Cargo.toml",
            'exe': "./rust-bench/target/release/bench"
      },
      'c' : {
            'build':"cd c-bench; cc main.c insrcdata.c -o ./target/bench ",
            'exe': "./c-bench/target/bench"
      },

}

lang = 'rust'
if len(sys.argv)>1:
      lang = sys.argv[1]

row_count = 500
if len(sys.argv)>2:
      row_count = int(sys.argv[2])
bench_data(row_count)

build_and_run(LANGS[lang])
