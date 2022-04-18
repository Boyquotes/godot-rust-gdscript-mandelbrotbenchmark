extends Node2D


var colors = PoolColorArray()


# Called when the node enters the scene tree for the first time.
func _ready():
	colors.append(Color(0.0,0.0,0.0))
	colors.append(Color(1.0,0.0,1.0))
	colors.append(Color(0.0,0.0,1.0))
	colors.append(Color(0.0,1.0,0.0))
	colors.append(Color(0.0,0.5,1.0))
	colors.append(Color(1.0,0.0,0.0))
	colors.append(Color(1.0,0.5,0.0))
	colors.append(Color(1.0,1.0,0.0))
	colors.append(Color(1.0,1.0,1.0))
	
	
func _draw():
	var msecsstart=OS.get_system_time_msecs()
	var c=Color(0.0,0.0,0.0)
	# from http://www.quitebasic.com/prj/math/mandelbrot/
	var l=100.0
	for i in range(0,500):
		for j in range(0,500):
			
			var u=i/250.0-1.5
			var v=j/250.0-1.0
			var x=u
			var y=v
			var n=0.0
			var r=x*x
			var q=y*y
			while (r+q)<4 && n<l:
				y=2*x*y+v
				x=r-q+u
				r=x*x
				q=y*y
				n=n+1
			if n<10: 
				c=colors[0]
			else:
				c = colors[int(round(8 * (n-10) / (l-10)))]
			draw_rect ( Rect2(Vector2(i,j), Vector2(1,1)), c, true, 1.0, false )
	var msecsstop=OS.get_system_time_msecs()
	print("gdscript result millisecs: ", msecsstop-msecsstart)
			
	
# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	#update()
#	pass

