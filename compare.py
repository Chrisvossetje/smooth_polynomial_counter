import json

def get_polynomails(file_path):
  polys = []
  with open(file_path) as file:
    for line in file.readlines():
      poly = parse_poly(line)
      polys.append(poly)
  return polys

def get_polynomails_set(file_path):
  polys = set()
  with open(file_path) as file:
    for line in file.readlines():
      poly = parse_poly(line)
      polys.add(poly)
  return polys

def parse_poly(line):
  parts = line.split(" | ")
  repr = parts[0]
  size = parts[1]
  # defined_points = list(map(int, parts[2][1:-2].split(", ")))
  if 'X' in repr:
    repr = fix_poly_repr(repr)
  
  return repr

def fix_poly_repr(old_repr):
  terms = old_repr.split(" + ")
  new_terms = []
  for t in terms:
    degs = t.split(" ")
    x = degs[0][-1]
    y = degs[1][-1]
    z = degs[2][-1]
    new_terms.append("1_"+x+y+z)
  
  new_str = new_terms[0]
  for t in new_terms[1:]:
    new_str += " " + t
  return new_str
    

def compare_polys(polys1, polys2):
  for p1 in polys1:
    polys2.remove(p1)
  return polys2

def poly_match(poly, poly_list):
  for other in poly_list:
    if poly == other:
      return True
  return False

polys1 = get_polynomails("input/2.txt")
polys2 = get_polynomails_set("input/1.txt")
print("read")
output = compare_polys(polys1, polys2)
with open("output.txt","w") as file:
  file.write(str(output))
print(output)