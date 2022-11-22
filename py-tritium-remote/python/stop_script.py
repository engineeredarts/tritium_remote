import os
import asyncio
import tritium_remote

SCRIPT_PATH = "start_stop.py"


async def main():
    auth_token = os.environ["TRITIUM_AUTH_TOKEN"]

    print("connecting...")
    tritium = await tritium_remote.connect("ws://localhost:1234", auth_token)
    print(f"stopping script {SCRIPT_PATH}...")
    await tritium.stop_script(SCRIPT_PATH)
    print("done.")


asyncio.run(main())
