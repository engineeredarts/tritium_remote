import os
import asyncio
import py_tritium_remote


async def main():
    auth_token = os.environ["TRITIUM_AUTH_TOKEN"]

    print("connecting...")
    tritium = await py_tritium_remote.connect("ws://localhost:1234", auth_token)

    print("querying system info...")
    system_info = await tritium.query_system_info()

    print("System info:")
    print(f"  serial:  {system_info.serial}")
    print(f"  name:    {system_info.name}")
    print(f"  version: {system_info.version}")


asyncio.run(main())
