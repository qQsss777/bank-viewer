import os
import shutil

packages_spec = ["user-service"]
file_copied = shutil.copyfile("template.openapi.yaml", "openapi.yaml")
with open("openapi.yaml", mode="a") as root:
  for entry in packages_spec:
    path_doc = os.path.join("../..", entry, "docs/specs/openapi.yaml")
    with open(path_doc) as micro_service:
      paths_data = False
      while True:
        line = micro_service.readline()
        if not line:
          break
        if paths_data:
            root.write(line)
        if line.startswith("paths:"):
          paths_data = True
          root.write("\n")
          root.write(line)
        
shutil.move("openapi.yaml", "../../gateway/docs/specs/openapi.yaml")