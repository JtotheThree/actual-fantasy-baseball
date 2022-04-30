import { gql } from "@apollo/client";

export const MY_TEAM_FOR_LEAGUE = gql`
query MyTeamForLeague (
  $ownerId: ID!
  $leagueId: ID!
) {
  teams(filter:{
    owner: $ownerId
    league: $leagueId
  }) {
    id
    name
    gold
    owner {
      id
      username
    }
    players {
      id
      name
    }
  }
}
`;

//export const GET_TEAM = gql`
//
//`
