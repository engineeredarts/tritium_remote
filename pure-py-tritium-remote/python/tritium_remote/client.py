import asyncio
import websockets
import json

# websocket.enableTrace(True)


class TritiumRemoteClient:
    def __init__(self, url, auth_token, description):
        self._url = url
        self._auth_token = auth_token
        self._description = description
        self._next_request_id = 0
        self._queries_by_id = {}

    @property
    def headers(self):
        auth_token = self._auth_token
        metadata = json.dumps(
            {"session_type": "graphql", "description": self._description}
        )
        return {"x-tritium-token": auth_token, "x-tritium-session-metadata": metadata}

    async def connect(self):
        ws = await websockets.connect(self._url, extra_headers=self.headers)
        self._ws = ws

        async def listen():
            while True:
                msg = await ws.recv()
                self._on_message(msg)

        self._listen_task = asyncio.create_task(listen())


    async def query(self, document, variables=None):
        request_id = self._next_request_id
        self._next_request_id += 1

        await self._send_graphql_message(request_id, document, variables)

        loop = asyncio.get_running_loop()
        future = loop.create_future()

        self._queries_by_id[request_id] = future

        # wait for response
        await future

        return future.result()

    async def _send_graphql_message(self, request_id, document, variables):

        msg = json.dumps(
            {
                "type": "graphql",
                "auth_token": self._auth_token,
                "request_id": request_id,
                "document": document,
                "variables": variables,
            }
        )

        await self._ws.send(msg)

    def _on_message(self, msg):
        print("MESSAGE", msg)

        try:
            m = json.loads(msg)
        except Exception as e:
            print("failed to decode message JSON", e) 
            return

        try:
            message_type = m["type"]
            data = m["data"]
            request_id = m["request_id"]
        except KeyError as e:
            print("bad message", e) 
            return

        if message_type == "graphql_response":
            self._on_graphql_response(request_id, data)
        else:
            print("unrecognised message type:", message_type)

    def _on_graphql_response(self, request_id, data):
        try: 
            query = self._queries_by_id.pop(request_id)
            query.set_result(data)
        except KeyError:
            pass

