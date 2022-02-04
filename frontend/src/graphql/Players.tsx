import { gql } from "@apollo/client";

export const GET_PLAYERS_FOR_DRAFTING = gql`
query GetPlayersForDrafting (
  $league: String!
) {
  players(filter: {
    league: $league
  }) {
    id
    name
    cost
    gender
    race
    class
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
`;

export default function FilterBuilder() {

}
