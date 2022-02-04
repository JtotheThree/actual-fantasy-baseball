import { useQuery } from "@apollo/client"
import { GET_PLAYERS_FOR_DRAFTING } from "../graphql/Players"
import { Card, CardBody, CardHeader } from "./Card";
import Loader from "./Loader";

type DraftingPlayerProps = {
  player: any,
}

function DraftingPlayer(props: DraftingPlayerProps) {
  return (
    <Card>
      <CardHeader title={props.player.name} />
      <CardBody>
        <p>Cost: {props.player.cost}</p>
        <p>{props.player.class} {props.player.race} {props.player.gender}</p>
        <table className="table-auto">
          <thead className="bg-gray-700 text-paper">
          <tr>
            <th>STR</th>
            <th>DEX</th>
            <th>CON</th>
            <th>INT</th>
            <th>WIS</th>
            <th>CHR</th>
            <th>HEALTH</th>
          </tr>
          </thead>
          <tbody>
          <tr>
            <td>{props.player.strength}</td>
            <td>{props.player.dexterity}</td>
            <td>{props.player.constitution}</td>
            <td>{props.player.intelligence}</td>
            <td>{props.player.wisdom}</td>
            <td>{props.player.charisma}</td>
            <td>{props.player.maxHealth}</td>
          </tr>
          </tbody>
        </table>
        {props.player.traits}
      </CardBody>
    </Card>
  )
}

type DraftingProps = {
  league: any,
}

export default function Drafting(props: DraftingProps) {
  let { loading, error, data } = useQuery(GET_PLAYERS_FOR_DRAFTING, {
    variables: {league: props.league.id}
  });

  if (loading) {
    return (
      <div className="flex flex-col md:w-1/1 mx-auto p-8 space-y-8">
        <Loader />
      </div>
    )
  }

  if (error) {
    return (
      <div>{error}</div>
    )
  }

  console.log(data);

  const players = data.players.map((player: any) => {
    return (
      <tr>
        <td className="text-2xl">{player.name}</td>
        <td>{player.class}</td>
        <td>{player.race}</td>
        <td>{player.strength}</td>
        <td>{player.dexterity}</td>
        <td>{player.constitution}</td>
        <td>{player.wisdom}</td>
        <td>{player.intelligence}</td>
        <td>{player.charisma}</td>
        <td>{player.maxHealth}</td>
        <td>{player.traits}</td>
        <td>{player.cost}</td>
      </tr>
    )
    /*return (
      <DraftingPlayer key={player.id} player={player} />
    )*/
  })

  return (
    <Card>
      <CardHeader title={"Drafting"} />
      <CardBody>
        <table className="min-w-full divide-y divide-gray-200">
          <thead>
            <tr>
              <th className="text-left">NAME</th>
              <th className="text-left">CLASS</th>
              <th className="text-left">RACE</th>
              <th className="text-left">STR</th>
              <th className="text-left">DEX</th>
              <th className="text-left">CON</th>
              <th className="text-left">WIS</th>
              <th className="text-left">INT</th>
              <th className="text-left">CHR</th>
              <th className="text-left">HP</th>
              <th className="text-left">TRAITS</th>
              <th className="text-left">COST</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200">
            {players}
          </tbody>
        </table>
      </CardBody>
    </Card>
  )
}
