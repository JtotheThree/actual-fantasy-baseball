import { gql, useApolloClient, useMutation, useQuery } from "@apollo/client"
import { useNavigate } from "react-router-dom";
import { useSetRecoilState } from "recoil";
import { selectedLeagueState } from "../components/App";
import { Card, CardBody, CardFooter, CardHeader } from "../components/Card";
import { MY_ID, SELECTED_LEAGUE } from "../constant";
import { JOIN_LEAGUE } from "../graphql/league";


const PUBLIC_LEAGUES = gql`
query Leagues (
  $excludeId: ID!
) {
  leagues(filter: {
    _and: [
      {public: true},
      {owner: {
      	_ne: $excludeId
      }},
      {managers: {
        _ne: $excludeId
      }}
    ]
  }) {
    id
    name
    description
    maxPlayers,
    state,
    manualState,
    managersCount,
    owner {
      username
    }
    managers {
      username
    }
  }
}
`;

export default function JoinLeague() {
  const my_id = localStorage.getItem(MY_ID);
  const { loading, error, data } = useQuery(PUBLIC_LEAGUES, {
    fetchPolicy: "network-only",
    variables: {
      excludeId: my_id,
    }
  });

  const setSelectedLeague = useSetRecoilState(selectedLeagueState);

  const navigate = useNavigate();
  const client = useApolloClient();

  const [ joinLeague ] = useMutation(JOIN_LEAGUE, {
    fetchPolicy: "network-only",
    onCompleted: ({ joinLeague }) => {
      localStorage.setItem(SELECTED_LEAGUE, joinLeague.id);
      setSelectedLeague(joinLeague.id);
      client.resetStore();
      navigate(`/league/${joinLeague.id}`)
    },
    onError: ({message}) => {
      console.error(message);
    }
  });

  if (loading) {
    return (
      <div className="flex flex-col md:w-1/3 mx-auto p-8 space-y-8">
        Loading...
      </div>
    )
  }

  if (error) {
    return (
      <div className="flex flex-col md:w-1/3 mx-auto p-8 space-y-8">
        {error}
      </div>
    )
  }

  if (data.leagues.length === 0) {
    return (
      <div className="flex flex-col md:w-1/3 mx-auto p-8 space-y-8">
        No public leagues available to join
      </div>
    )
  }

  function handleJoin(id: string) {
    joinLeague({variables: {id: id}});
  };

  const leagues = data.leagues.map((league: any) => {
    return (
      <Card key={league.id}>
        <CardHeader title={league.name}/>
        <CardBody>
          Owner: {league.owner.username} <br/>
          Description: {league.description} <br/>
        </CardBody>
        <CardFooter>
          <button
            className="align-self-end btn py-1 px-8 bg-gray-700 text-paper rounded-sm border-b-4 border-paper font-bold hover:bg-red-800 float-right"
            onClick={() => handleJoin(league.id)}
          >
            Join
          </button>
        </CardFooter>
      </Card>
    )
  })

  return (
    <div className="flex flex-col md:w-1/3 mx-auto p-8 space-y-8">
      {leagues}
    </div>
  )
}