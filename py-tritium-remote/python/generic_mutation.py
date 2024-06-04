#!/usr/bin/env python3

import os
import asyncio
import json
import tritium_remote

SCRIPT_PATH = "start_stop.py"


async def main():
    auth_token = os.environ["TRITIUM_AUTH_TOKEN"]
    host = os.environ.get("TRITIUM_HOST", "localhost")

    print("connecting...")
    tritium = await tritium_remote.connect(
        f"ws://{host}:1234", auth_token, "Python tritium-remote example - start script"
    )

    document = """
        mutation trigger($input:ScriptTriggerInput!) {
            manuallyTriggerScript(input: $input) {
                script {
                    status
                }
            }
        }   
    """

    variables = {"input": {"action": "START", "path": "start_stop.py"}}

    response = await tritium.query(document, json.dumps(variables))
    print("Response: ", response)


asyncio.run(main())
