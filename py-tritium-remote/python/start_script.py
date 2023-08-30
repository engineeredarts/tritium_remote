import os
import asyncio
import tritium_remote

SCRIPT_PATH = "start_stop.py"


async def main():
    auth_token = os.environ["TRITIUM_AUTH_TOKEN"]
    host = os.environ.get("TRITIUM_HOST", "localhost")

    print("connecting...")
    tritium = await tritium_remote.connect(f"ws://{host}:1234", auth_token, , "Python tritium-remote example - start script")

    print(f"starting script {SCRIPT_PATH}...")
    await tritium.start_script(SCRIPT_PATH)
    print("done.")


asyncio.run(main())
