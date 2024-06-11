#!/usr/bin/env python3

import os
import asyncio
import tritium_remote

# full path to the sequence on the robot
PROJECT_PATH = "/var/opt/tritium/content/test_sequence.project"


async def main():
    auth_token = os.environ["TRITIUM_AUTH_TOKEN"]
    host = os.environ.get("TRITIUM_HOST", "localhost")

    print("connecting...")
    tritium = await tritium_remote.connect(
        f"ws://{host}:1234", auth_token, "Python tritium-remote example - play sequence"
    )

    print(f"playing sequence {PROJECT_PATH}...")
    await tritium.play_sequence(PROJECT_PATH)
    print("done.")


asyncio.run(main())
