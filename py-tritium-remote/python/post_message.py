import os
import asyncio
import tritium_remote
from time import time
import json


async def main():
    auth_token = os.environ["TRITIUM_AUTH_TOKEN"]
    host = os.environ.get("TRITIUM_HOST", "localhost")

    print("connecting...")
    tritium = await tritium_remote.connect(f"ws://{host}:1234", auth_token)

    while True:
        t = time()
        message = {
            "python_remote_time": f"The (python) remote time is now {t}s since the start of 1970"
        }
        print(f'posting to channel "talking_clock": {message}')
        message_json = json.dumps(message)
        await tritium.post_message("talking_clock", message_json)
        await asyncio.sleep(1)


asyncio.run(main())
