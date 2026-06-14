import os

apps = os.listdir("src/bin")
apps.sort()

for app in apps:
    app = app[: app.find(".")]
    os.system("cargo build --bin %s --release" % app)
    print("[build.py] application %s built as ELF" % app)
