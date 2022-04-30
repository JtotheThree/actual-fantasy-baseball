import {useQuery} from "@apollo/client";
import {useParams} from "react-router-dom"
import {Card, CardHeader, CardBody} from "../components/Card";
import Loader from "../components/Loader";
import Roster from "../components/Roster";
import {GET_TEAM} from "../graphql/Team";

export default function Team() {
  let { id } = useParams();

  let { loading, error, data } = useQuery(GET_TEAM, {
    variables: {
      id: id
    }
  });

  if ( loading ) {
    return (
      <div>
        <Loader />
      </div>
    )
  }

  if ( error ) {
    return (
      <div>
        { error }
      </div>
    )
  }

  let team = data.team;

  return (
    <div className="md:w-1/1 mx-auto p-8 space-y-8 justify-center items-center h-screen">
      <div className="md:w-2/3">
        <h1 className="text-4xl h-5 text-center border-b-2 border-gray-700 tracking-wid px-4 font-title">
          <span className="bg-paper px-8">
            {team.name}
          </span>
        </h1>
      </div>
      <div className="flex flex-row p-2">
        <Card>
          <CardHeader title="Roster" />
          <CardBody>
            <Roster roster={team.roster} />
          </CardBody>
        </Card>
        <Card>
          <CardHeader title="Lineup" />
          <CardBody>

          </CardBody>
        </Card>
      </div>
    </div>
  )
}
