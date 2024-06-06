from .client import TritiumRemoteClient


async def connect(url, auth_token, description=None):
    print(f"[tritium_remote] CONNECT {url}")

    client = TritiumRemoteClient(url, auth_token, description)

    await client.connect()

    return client
