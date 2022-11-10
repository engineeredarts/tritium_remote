import asyncio
import py_tritium_remote

print("sum_as_string", py_tritium_remote.sum_as_string(1, 1))


async def main():
    print("awaiting call_rust_sleep...")
    await py_tritium_remote.call_rust_sleep()
    print("...done")


asyncio.run(main())
