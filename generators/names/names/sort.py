import glob
import os.path as osp

if __name__ == '__main__':
    for filename in glob.glob(osp.join("./data", '*.txt')):
        names = []

        with open(filename) as f:
            names = [line.strip() for line in f]

        # Ensure no duplicates

        cleaned = []
        [cleaned.append(x) for x in names if x not in cleaned]

        sorted_names = sorted(cleaned)

        with open(filename, 'w') as f:
            f.truncate(0)
            for name in sorted_names:
                f.write(f"{name}\n")
