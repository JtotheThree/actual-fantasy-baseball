import numpy as np
import torch
from torch.distributions import OneHotCategorical
from torchvision.transforms import Compose

from data import Vocabulary, OneHot, Groups, Races, ToTensor
from utils import load_model


class Generator:
    """Base Generator class that can load trained model and require every subclass to implement `generate` method"""
    def __init__(self, model_path, device="cpu"):
        self.model = load_model(model_path, device=device)
        self.device = device

    def generate(self, num_samples):
        raise NotImplementedError


class RNNCellGenerator(Generator):
    def __init__(self, model_path, device="cpu"):
        super().__init__(model_path, device)

        self.vocab = Vocabulary()
        self.races = Races()
        self.groups = Groups()
        self.to_tensor = ToTensor()

        self.name_transform = Compose([self.vocab, OneHot(self.vocab.size), ToTensor()])
        self.race_transform = Compose([self.races, OneHot(self.races.size), ToTensor()])
        self.group_transform = Compose([self.groups, OneHot(self.groups.size), ToTensor()])

    def _init_random_input(self):
        """Helper function that initialize random letter, race and group"""
        letter = np.random.choice(self.vocab.start_letters)
        race = np.random.choice(self.races.available_races)
        group = np.random.choice(self.groups.available_groups)

        return letter, race, group

    def _transform_input(self, letter, race, group):
        """Helper function to transform input into tensors"""
        letter_tensor = self.name_transform(letter).to(self.device)
        race_tensor = self.race_transform(race).to(self.device)
        group_tensor = self.group_transform(group).to(self.device)

        return letter_tensor, race_tensor, group_tensor

    def generate(self, num_samples):
        with torch.no_grad():
            print("_" * 20)
            for _ in range(num_samples):
                hx, cx = self.model.init_states(batch_size=1, device=self.device)

                letter, race, group = self._init_random_input()
                letter_t, race_t, group_t = self._transform_input(letter, race, group)

                input = torch.cat([letter_t, race_t, group_t], 1)
                outputs = [letter]

                while True:
                    output, hx, cx = self.model(input, hx, cx)

                    sample = OneHotCategorical(logits=output).sample()
                    index = torch.argmax(sample)
                    char = self.vocab.idx2char[index.item()]
                    outputs.append(char)

                    input = torch.cat([sample, race_t, group_t], 1)

                    if char == '.' or len(outputs) == 50:
                        break

                print("Start letter: {}, Race: {}, Group: {}".format(letter, race, group))
                print("Generated sample: {}".format(''.join(map(str, outputs))))

            print("_" * 20)


class RNNLayerGenerator(Generator):
    def __init__(self, model_path, device="cpu", max_len=50, verbose=1):
        super().__init__(model_path, device)
        self.max_len = max_len
        self.verbose = verbose

        self.vocab = Vocabulary()
        self.races = Races()
        self.groups = Groups()
        self.to_tensor = ToTensor()

        self.name_transform = Compose([self.vocab, OneHot(self.vocab.size), ToTensor()])
        self.race_transform = Compose([self.races, OneHot(self.races.size), ToTensor()])
        self.group_transform = Compose([self.groups, OneHot(self.groups.size), ToTensor()])

    def _init_random_input(self):
        """Helper function that initialize random letter, race and group"""
        letter = np.random.choice(self.vocab.start_letters)
        race = np.random.choice(self.races.available_races)
        group = np.random.choice(self.groups.available_groups)

        return letter, race, group

    def _transform_input(self, letter, race, group):
        """Helper function to transform input into tensors"""
        letter_tensor = self.name_transform(letter).to(self.device)
        race_tensor = self.race_transform(race).to(self.device)
        group_tensor = self.group_transform(group).to(self.device)

        return letter_tensor, race_tensor, group_tensor

    def _expand_dims(self, *tensors):
        """Add dimension along 0-axis to tensors"""
        return [torch.unsqueeze(t, 0) for t in tensors]

    def sample(self, letter, race, group):
        """Sample name from start letter, race and group"""
        with torch.no_grad():
            assert letter in self.vocab.start_letters, "Invalid letter"
            assert race in self.races.available_races, "Invalid race"
            assert group in self.groups.available_groups, "Invalid group"

            # Prepare inputs
            letter_t, race_t, group_t = self._transform_input(letter, race, group)
            letter_t, race_t, group_t = self._expand_dims(letter_t, race_t, group_t)

            # Merge all input tensors
            input = torch.cat([letter_t, race_t, group_t], 2)
            outputs = [letter]

            # Initialize hidden states
            hx, cx = self.model.init_states(batch_size=1, device=self.device)

            while True:
                output, hx, cx = self.model(input, hx, cx, lengths=torch.tensor([1]))

                sample = OneHotCategorical(logits=output).sample()
                index = torch.argmax(sample)
                char = self.vocab.get_char(index.item())

                if char == '.' or len(outputs) == self.max_len:
                    break

                outputs.append(char)
                input = torch.cat([sample, race_t, group_t], 2)

            name = ''.join(map(str, outputs))
            return name

    def generate(self, num_samples, race, gender):
        """Sample random names"""
        gen_names = []
        for _ in range(num_samples):
            first_letter, _, _ = self._init_random_input()
            last_letter, _, _ = self._init_random_input()

            gen_first = self.sample(first_letter, race, gender)
            gen_last = self.sample(last_letter, race, 'last')

            if len(gen_first) == 1 or len(gen_last) == 1 and race != "goblin":
                name = self.generate(1, race, gender)
                gen_names.append(name)
                continue

            gen_names.append([f"{gen_first} {gen_last}", race, gender])

        return gen_names


if __name__ == '__main__':
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("-mp", "--model_path")
    args = parser.parse_args()

    dnd = RNNLayerGenerator(model_path="./models/rnn_layer_epoch_150.pt")
    names = dnd.generate(500, 'orc', 'female')

    names = sorted(names)

    for name in names:
        print(f"{name[0]} \t\t{name[1]} {name[2]}")

    duplicates = 0
    cleaned = []

    for x in names:
        if x not in cleaned:
            cleaned.append(x)
        else:
            duplicates += 1

    print(f"Duplicates: {duplicates}")