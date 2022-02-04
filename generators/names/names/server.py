import logging
from fastapi import FastAPI
import uvicorn

from generator import RNNLayerGenerator

app = FastAPI()

nn = RNNLayerGenerator(model_path="./models/rnn_layer_epoch_150.pt")

@app.get('/name')
async def name(race, gender):
    name = nn.generate(1, race, gender)

    return {"name": name[0][0]}

if __name__ == "__main__":


    uvicorn.run(app, host="0.0.0.0", port=9000, log_level="warning")