import argparse
from pprint import pprint
from random import randrange

from gql import Client, gql
from gql.transport.aiohttp import AIOHTTPTransport

transport = AIOHTTPTransport(url="http://localhost:4000/")
client = Client(transport=transport, fetch_schema_from_transport=True)

query = gql(
    """
    query GetPlayers {
        players {
            id
            name
            cost
            gender
            race
            class
            handedness
            maxHealth
            strength
            dexterity
            constitution
            intelligence
            wisdom
            charisma
            traits
        }
    }
    """
)

def find_batter(players, name):
    for player in players:
        if player["name"] == name:
            return player

def find_pitchers(players):
    pitchers = []

    for player in players:
        if player["strength"] >= 12 and player["dexterity"] >= 14:
            pitchers.append(player)

    return pitchers

def calculate_modifier(stat):
   return round((stat - 10) / 2)

def run_simulation(batter, pitchers, count):
    hits = 0
    for _ in range(count):
        pitcher = pitchers[randrange(len(pitchers))]

        batter_roll = randrange(20)
        pitcher_roll = randrange(20)

        batter_mod = calculate_modifier(batter["dexterity"] + 2)
        pitcher_mod = calculate_modifier(pitcher["dexterity"]+pitcher["strength"])

        hit = False

        if batter_roll + batter_mod >= pitcher_roll + pitcher_mod:
            hit = True
            hits += 1

        #pprint(f"Batter: {batter_roll}+{batter_mod}, Pitcher: {pitcher_roll}+{pitcher_mod} = Result: {hit}")

    return hits

def find_batter_with_dex(players, dexterity):
    for player in players:
        #pprint(player)
        if player["dexterity"] == dexterity:
            return player

    print(f"No player found with dexterity {dexterity}")
    return None

def find_batters(players):
    return {
        "3": find_batter_with_dex(players, 3),
        "6": find_batter_with_dex(players, 6),
        "8": find_batter_with_dex(players, 8),
        "10": find_batter_with_dex(players, 10),
        "12": find_batter_with_dex(players, 12),
        "14": find_batter_with_dex(players, 14),
        "16": find_batter_with_dex(players, 16),
        "18": find_batter_with_dex(players, 18),
        "20": find_batter_with_dex(players, 20)
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("-b", "--batter", help="The name of the batter to test with")
    parser.add_argument("-c", "--count", help="How many simulations to run", default=200, type=int)
    args = parser.parse_args()

    players = client.execute(query)
    pitchers = find_pitchers(players["players"])

    if args.batter:
        batter = find_batter(players["players"], args.batter)
        hits = run_simulation(batter, pitchers, args.count)

        average = hits / args.count
        print(f"Hits: {hits} : Average {average}")

    else:
        for desc, batter in find_batters(players["players"]).items():
            if batter is None:
                print(f"{desc} : None available")
                continue
            hits = run_simulation(batter, pitchers, args.count)
            average = hits / args.count
            print(f"{desc} : {average}")