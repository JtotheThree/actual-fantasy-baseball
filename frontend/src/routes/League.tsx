import { useQuery } from "@apollo/client";
import { useParams } from "react-router-dom"
import { Card, CardHeader, CardSubHeader, CardBody } from "../components/Card";
import { GET_LEAGUE } from "../graphql/league";

function LeagueInfo() {
  let { id } = useParams();
  let { loading, error, data } = useQuery(GET_LEAGUE, {
    variables: {id: id}
  });

  if (loading) {
    return (
      <div>Loading...</div>
    )
  }

  if (error) {
    return (
      <div>{error}</div>
    )
  }

  let players = data.league.managers.map((manager: any) => {
    return (
      <li key={manager.username}>{manager.username}</li>
    )
  });

  const content = (
    <div>
      Owner: {data.league.owner.username}<br/><br/>

      Players: <ul>{players}</ul>

      <br/><br/>

      Public: {data.league.public.toString()}<br/>
      Max Players: {data.league.maxPlayers}<br/>
      State: {data.league.state}<br/>
      Manual: {data.league.manualState.toString()}<br/>
    </div>
  );

  return (
    <Card>
      <CardHeader title={data.league.name + " Info"} />
      <CardSubHeader content={data.league.description} />
      <CardBody>
        {content}
      </CardBody>
    </Card>
  )
}

function TeamInfo() {
  let { id } = useParams();


}

export default function League() {


  return (
    <div className="flex flex-col md:w-1/1 mx-auto p-8 space-y-8">
      <LeagueInfo />
    </div>
  )
}