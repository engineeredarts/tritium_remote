import asyncio
import py_tritium_remote

print("sum_as_string", py_tritium_remote.sum_as_string(1, 1))


async def main():
    print("awaiting connect...")
    # await py_tritium_remote.connect("ws://localhost:1234")
    await py_tritium_remote.connect()
    print("...done")


asyncio.run(main())
