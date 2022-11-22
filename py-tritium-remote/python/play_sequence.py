import os
import asyncio
import tritium_remote

PROJECT_PATH = "test_sequence.project"


async def main():
    auth_token = os.environ["TRITIUM_AUTH_TOKEN"]

    print("connecting...")
    tritium = await tritium_remote.connect("ws://localhost:1234", auth_token)

    print(f"playing sequence {PROJECT_PATH}...")
    await tritium.play_sequence(PROJECT_PATH)
    print("done.")


asyncio.run(main())
