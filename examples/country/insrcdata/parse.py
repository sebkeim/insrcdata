import csv

v = "name", "alpha-2", "alpha-3", "country-code", "iso_3166-2", "region", "sub-region", "intermediate-region", "region-code", "sub-region-code", "intermediate-region-code"

SR_COLS = "sub-region", "sub-region-code", "region-code"
subregions=set([''])
csr = csv.writer(open("subregion.csv", "w"))
csr.writerow(SR_COLS)

R_COLS = "region", "region-code"
regions=set([''])
cr = csv.writer(open("region.csv", "w"))
cr.writerow(R_COLS)


f = csv.reader(open("countries.csv"))



cols = next(f)
for  row in f:
      d = dict(zip(cols,row))
      if d["sub-region-code"] not in subregions:
            csr.writerow(d[x] for x in SR_COLS)
            subregions.add(d["sub-region-code"])
      if d["region-code"] not in regions:
            cr.writerow(d[x] for x in R_COLS)
            regions.add(d["region-code"])
      
