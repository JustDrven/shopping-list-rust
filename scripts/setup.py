import os.path
import shutil
import util

EXAMPLE_ENV = ".example.env"
ENV = ".env"

if os.path.exists(ENV):
    util.log("The environment file already exists!")
    exit(1)

shutil.copy(EXAMPLE_ENV, ENV)
util.log("The environment file copied!")
