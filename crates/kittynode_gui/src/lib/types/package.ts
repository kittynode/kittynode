export type Package = {
  package: PackageInfo;
  containers: Container[];
};

export type PackageInfo = {
  name: string;
  version: string;
};

export type Container = {
  image: string;
};
