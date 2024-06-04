#!/usr/bin/env python3

import os
import asyncio
import tritium_remote


async def main():
    auth_token = os.environ["TRITIUM_AUTH_TOKEN"]
    host = os.environ.get("TRITIUM_HOST", "localhost")

    print("connecting...")
    tritium = await tritium_remote.connect(
        f"ws://{host}:1234", auth_token, "Python tritium-remote example - list hosts"
    )

    document = """
        query { 
            system { 
                serial
                hosts {
                    name
                    cpuCount
                } 
            }
        }
    """

    response = await tritium.query(document, None)
    print("Response: ", response)


asyncio.run(main())
