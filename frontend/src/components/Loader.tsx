const Loader = () => {
  let circleCommonClasses = 'h-2.5 w-2.5 bg-gray-600 rounded-full';

  return (
  <div className='flex items-center justify-center'>
        <div className={`${circleCommonClasses} mr-1 animate-bounce`}></div>
        <div className={`${circleCommonClasses} mr-1 animate-[bounce_1s_infinite_200ms]`}></div>
        <div className={`${circleCommonClasses} animate-[bounce_1s_infinite_400ms]`}></div>
  </div>
  );
};

export default Loader;
