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
    league {
      id
      name
      owner {
        id
        username
      }
    }
    gold
    roster {
      startingPitcher {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      reliefPitchers {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      catcher {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      catcherReserves {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      firstBase {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      secondBase {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      thirdBase {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      shortstop {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      infieldReserves {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      leftField {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      centerField {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      rightField {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      outfieldReserves {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
    }
    lineup {
      first
      second
      third
      fourth
      fifth
      sixth
      seventh
      eighth
      ninth
    }
    players {
      id
      name
      class
      race
      strength
      dexterity
      constitution
      wisdom
      intelligence
      charisma
      traits
      cost
    }
  }
}
`;

export const GET_TEAM = gql`
query GetTeam(
  $id: ID!
) {
team(id: $id) {
    id
    name
    owner {
      id
      username
    }
    league {
      id
      name
      owner {
        id
        username
      }
    }
    gold
    roster {
      startingPitcher {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      reliefPitchers {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      catcher {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      catcherReserves {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      firstBase {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      secondBase {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      thirdBase {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      shortstop {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      infieldReserves {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      leftField {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      centerField {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      rightField {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
      outfieldReserves {
        name
        gender
        class
        race
        handedness
        strength
        dexterity
        constitution
        wisdom
        intelligence
        charisma
        maxHealth
        traits
      }
    }
    lineup {
      first
      second
      third
      fourth
      fifth
      sixth
      seventh
      eighth
      ninth
    }
    players {
      id
      name
      class
      race
      strength
      dexterity
      constitution
      wisdom
      intelligence
      charisma
      traits
      cost
    }
  }
}
`;
