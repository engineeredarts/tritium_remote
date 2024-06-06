#!/usr/bin/env python3

import os
import asyncio
import tritium_remote


def on_response(response):
    print("Response:", response)


async def main():
    auth_token = os.environ["TRITIUM_AUTH_TOKEN"]
    host = os.environ.get("TRITIUM_HOST", "localhost")

    print("connecting...")
    tritium = await tritium_remote.connect(
        f"ws://{host}:1234",
        auth_token,
        "Python tritium-remote example - generic subscription (world events)",
    )

    document = """
        subscription {
            worldEvents {
                timestamp
                eventData
            }
        }
    """

    subscription = await tritium.subscribe(document, None, on_response)
    print("Subscription:", subscription)


asyncio.run(main())
